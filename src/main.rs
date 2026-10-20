use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Orientation};
use simple_transcribe_rs::model_handler;

const APP_ID: &str = "com.rodrigues.gtk-rust";
const APP_TITLE: &str = "GTK4 Hyprland Rust";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    let result = (|| -> Result<(), std::boxed::Box<dyn std::error::Error>> {
        let filename = record_audio()?;
        let runtime = tokio::runtime::Runtime::new()?;
        let text = runtime.block_on(transcribe(filename))?;
        println!("{text}");
        Ok(())
    })();
    if let Err(error) = result {
        eprintln!("Erro ao gravar ou transcrever áudio: {error}");
    }

    app.run()
}

async fn transcribe(filename: &str) -> Result<String, std::boxed::Box<dyn std::error::Error>> {
    let model = model_handler::ModelHandler::new("tiny", "models/").await;
    let model_path = model.get_model_dir();
    if fs::metadata(&model_path).map_or(true, |metadata| !metadata.is_file() || metadata.len() == 0)
    {
        return Err(format!(
            "Modelo indisponível em {model_path}. Verifique a conexão e a permissão de escrita em models/."
        )
        .into());
    }
    let transcriber = simple_transcribe_rs::transcriber::Transcriber::new(model);

    let output = transcriber.transcribe(filename, None)?;

    Ok(output.get_text().to_owned())
}

fn record_audio() -> Result<&'static str, std::boxed::Box<dyn std::error::Error>> {
    let filename: &str = "recording.wav";
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("no input device connected")?;

    let supported_config = device.default_input_config()?;
    let sample_format = supported_config.sample_format();
    let config: cpal::StreamConfig = supported_config.into();

    let spec = hound::WavSpec {
        channels: config.channels,
        sample_rate: config.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = hound::WavWriter::create(filename, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    let stream = match sample_format {
        cpal::SampleFormat::I16 => build_recording_stream::<i16>(&device, config, &writer),
        cpal::SampleFormat::U16 => build_recording_stream::<u16>(&device, config, &writer),
        cpal::SampleFormat::F32 => build_recording_stream::<f32>(&device, config, &writer),
        format => return Err(format!("unsupported input sample format: {format}").into()),
    }?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(60));

    drop(stream);
    if let Some(writer) = writer.lock().unwrap().take() {
        writer.finalize()?;
    }

    Ok(filename)
}

fn build_recording_stream<T>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    writer: &Arc<Mutex<Option<hound::WavWriter<std::io::BufWriter<fs::File>>>>>,
) -> Result<cpal::Stream, cpal::Error>
where
    T: cpal::SizedSample,
    i16: cpal::FromSample<T>,
{
    let writer = Arc::clone(writer);
    device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            let mut guard = writer.lock().unwrap();
            if let Some(writer) = guard.as_mut() {
                for &sample in data {
                    writer.write_sample(sample.to_sample::<i16>()).unwrap();
                }
            }
        },
        |err| eprintln!("Stream error: {err}"),
        None,
    )
}

fn build_ui(app: &Application) {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(20)
        .margin_bottom(20)
        .margin_end(20)
        .margin_start(20)
        .build();

    content_box.set_cursor_from_name(Some("crosshair"));

    let click = gtk::GestureClick::new();
    click.connect_pressed(|_, _, _, _| println!("Clicked"));
    content_box.add_controller(click);

    let button = Button::with_label("Sample Label");
    button.connect_clicked(|btn| {
        btn.set_label("Hello");
        if let Err(e) = fs::write("foo.txt", "Hello World") {
            eprintln!("err to save file: {e}");
        }
    });

    content_box.append(&button);
    button.set_cursor_from_name(Some("pointer"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&content_box)
        .build();

    window.present();
}
