// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use palette::{FromColor, Gradient, Lch, Srgb};

#[tauri::command]
fn generate_gradient(r: u8, g: u8, b: u8) -> Vec<Vec<u8>> {
    /*
     * palette 0.6 버전 을 사용함 (cargo add pallete@0.6) : Gradient 가 0.7부터 없어짐
     */

    // 1. 입력 받은 RGB를 0.0 ~ 1.0 f32 표준화 및 Lch(Lightness) 변환
    let my_rgb: Srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let my_lch: Lch = Lch::from_color(my_rgb);

    // 2. Gradient 생성 : 최대 밝기 100.0
    let gradient = Gradient::new(vec![
        Lch::new(0.0, my_lch.chroma, my_lch.hue), // 가장 어두움 (0.0)
        my_lch,                                   // 원래 색상
        Lch::new(100.0, my_lch.chroma, my_lch.hue), // 가장 밝음 (100.0)
    ]);

    // 3. 그 중 10개의 색상을 뽑아 컴포넌트 분리 후, 0~255 범위의 u8 배열로 변환
    let colors = gradient
        .take(10)
        .map(|color| {
            // LCH -> Linear -> Standard RGB 순서로 변환
            let rgb_standard = Srgb::from_color(color);
            let (r_f, g_f, b_f) = rgb_standard.into_components();

            vec![
                (r_f * 255.0).round() as u8,
                (g_f * 255.0).round() as u8,
                (b_f * 255.0).round() as u8,
            ]
        })
        .collect::<Vec<_>>();
    dbg!(&colors); // Tauri 커맨드는 소유권을 반환해야 하므로 dbg!에는 참조(&)를 주는 것이 안전

    colors
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_gradient])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
