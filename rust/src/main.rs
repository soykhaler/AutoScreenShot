use raylib::prelude::*;
use screenshots::Screen;
use std::time::{Duration, Instant};

const OUTPUT_FILE: &str = "capture.jpg";

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(700, 300)
        .title("AutoScreenshot by SoyKhaler")
        .build();

    rl.set_target_fps(60);

    let mut interval: u64 = 15;
    let mut selected_monitor: usize = 0;

    let screens = Screen::all().unwrap();

    let mut last_capture = Instant::now();

    while !rl.window_should_close() {

        // Flecha derecha = aumentar intervalo
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            interval += 1;
        }

        // Flecha izquierda = disminuir intervalo
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {

            if interval > 1 {
                interval -= 1;
            }
        }

        // Flecha arriba = siguiente monitor
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {

            if selected_monitor + 1 < screens.len() {
                selected_monitor += 1;
            }
        }

        // Flecha abajo = monitor anterior
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {

            if selected_monitor > 0 {
                selected_monitor -= 1;
            }
        }

        // Captura automática
        if last_capture.elapsed() >= Duration::from_secs(interval) {

            match screens[selected_monitor].capture() {

                Ok(image) => {

                    image.save(OUTPUT_FILE).unwrap();

                    println!("Actualizado: {}", OUTPUT_FILE);
                }

                Err(err) => {

                    println!("Error capturando pantalla: {}", err);
                }
            }

            last_capture = Instant::now();
        }

        // Dibujar interfaz
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_text(
            "AutoScreenshot by SoyKhaler",
            20,
            20,
            30,
            Color::GREEN,
        );

        d.draw_text(
            &format!("Intervalo: {} segundos", interval),
            20,
            90,
            25,
            Color::WHITE,
        );

        d.draw_text(
            &format!(
                "Monitor seleccionado: {} / {}",
                selected_monitor,
                screens.len() - 1
            ),
            20,
            130,
            25,
            Color::WHITE,
        );

        d.draw_text(
            "← → Cambiar intervalo",
            20,
            200,
            20,
            Color::GRAY,
        );

        d.draw_text(
            "↑ ↓ Cambiar monitor",
            20,
            230,
            20,
            Color::GRAY,
        );

        d.draw_text(
            &format!("Guardando en: {}", OUTPUT_FILE),
            20,
            260,
            20,
            Color::YELLOW,
        );
    }
}
