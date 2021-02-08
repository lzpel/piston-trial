extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;

use std::rc::Rc;

use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

enum GameState {
    Title,
    Playing,
}


fn main() {
    let mut state = GameState::Title;
    // 組:tuple
    let (width, height) = (512, 320);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: sprite", (width, height))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .resizable(false)
            .build()
            .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let id;
    let mut scene = Scene::new();
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let logo = Rc::new(Texture::from_path(
        &mut texture_context,
        assets.join("rust.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap());
    let title: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("title.gif"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let pattern: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("pattern.gif"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let mut sprite = Sprite::from_texture(logo.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    id = scene.add_child(sprite);

    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
        Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
        Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
        Wait(0.5),
        Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Blink(1.0, 5)),
        While(Box::new(WaitForever), vec![
            Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
            Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
        ]),
    ]);
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseFunction::ExponentialInOut,
                             Box::new(RotateTo(2.0, 360.0))));
    scene.run(id, &rotate);

    println!("Press any key to pause/resume the animation!");

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            match state{
                GameState::Title => {
                    image(&title, c.transform, g);
                },
                GameState::Playing => {
                    Image::new().rect([0.0,0.0,32.0,32.0]).src_rect([0.0,0.0,32.0,32.0]).draw(&pattern,&DrawState::default(),c.transform,g );
                }
            }
            scene.draw(c.transform, g);
        });
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match state{
                GameState::Title => {
                    state = GameState::Playing;
                }
                _ => {},
            }
            if key == Key::P {
                scene.toggle(id, &seq);
                scene.toggle(id, &rotate);
            }
        }
    }
}
