// mod hoge;
// use hoge::fuga::{func01, func02};

mod rayt;
use crate::{rayt::*, shape::Shape};

use float3::Float3;
use rayt::camera::Camera;
use shape::{HitInfo, SimpleScene, Sphere};

use env_logger;
use log::{error, info, warn};

use rayon::prelude::*;

use winit::{
    //Event と WindowEvent という二つの型（または列挙型）が winit::event モジュールからインポート
    event::{Event, WindowEvent},
    //ControlFlow と EventLoop が winit::event_loop モジュールからインポート
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// use nalgebra::Float3;

// ライブラリのimport
// 外部のクレートから関数や構造体を使用するためには、useキーワードを使用する
// rustではライブラリをクレートと呼び、クレートの中の関数や構造体をモジュールと呼ぶ
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;

pub const sample: usize = 25;
pub const depth: usize = 25;

fn main() {
    env_logger::init();
    //event_loopの定義
    //これによって、ウィンドウ内での色々なイベントを取得できる
    let event_loop = EventLoop::new();
    // let screenWidth = 1280;
    // let screenHeight = 720;

    // let screenWidth = 640;
    // let screenHeight = 360;

    // let screenWidth = 426;
    // let screenHeight = 240;

    let screenWidth = 200;
    let screenHeight = 200;

    // let camera = Camera::from_lookat(
    //     Float3::new(0.5, 0.0, 0.0),
    //     Float3::new(0.5, 0.0, 1.0),
    //     Float3::new(0.0, 1.0, 0.0),
    //     90.0,
    //     16.0 / 9.0,
    // );
    // let ray = camera.ray(0.5, 0.5);
    // println!("{:?}", ray.direction.normalize());

    let camera = Camera::from_lookat(
        Float3::new(278.0, 278.0, -800.0),
        Float3::new(278.0, 278.0, 0.0),
        Float3::new(0.0, 1.0, 0.0),
        20.0,
        1.0,
    );

    //winitクレート(ライブラリ)を使用して、ウィンドウを作成する
    let window = WindowBuilder::new()
        .with_title("Simple Window")
        .with_inner_size(LogicalSize::new(screenWidth, screenHeight))
        .build(&event_loop)
        .unwrap();

    //作成されたウィンドウのサイズを取得する
    //作成時にサイズを指定していない場合は、デフォルトのサイズが返される
    let window_size = window.inner_size();

    //pixelsクレートのSurfaceTextureを使用してテクスチャの作成
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    //pixelsクレートのPixelsを使用して、画像バッファの作成
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    // let sphere = Sphere::new(Float3::new(0.0, -5.0, 0.5), 5.0);
    let scene = SimpleScene::new();

    //move |event, _, control_flow
    //この引数はクロージャと呼ばれるもので、関数のように使用できる
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        //いわゆるswitch文
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame();

                frame
                    .chunks_exact_mut(4)
                    .enumerate()
                    .par_bridge()
                    .for_each(|(i, pixel)| {
                        let u = (i % window_size.width as usize) as f64 / window_size.width as f64;
                        let v = 1.0
                            - (i / window_size.width as usize) as f64 / window_size.height as f64;

                        let ray = camera.ray(u, v);
                        let mut color = (0..sample)
                            .into_par_iter() // 並列イテレータ
                            .fold(
                                || Float3::new(0.0, 0.0, 0.0),
                                |acc, _| acc + scene.trace(ray, depth),
                            )
                            .reduce(|| Float3::new(0.0, 0.0, 0.0), |acc, val| acc + val);
                        color /= sample as f64;
                        // let c = scene.trace(ray, 25).gamma(2.2);
                        // let c = scene.trace(ray);

                        // println!("{:?}", c);

                        pixel[0] = (color.x() * 255.0) as u8;
                        pixel[1] = (color.y() * 255.0) as u8;
                        pixel[2] = (color.z() * 255.0) as u8;
                        pixel[3] = 255;
                    });
                //}

                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}
