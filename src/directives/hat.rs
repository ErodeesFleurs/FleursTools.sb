use crate::utils::{directives, image, template};

const HAT_TEMPLATE_FRAMES: [[&str; 1]; 1] = [["01"]];

const CHEST_DIRECTIVES: &str = "?setcolor=fff?replace;fff0=fff?crop;0;0;2;2?blendmult=/items/active/weapons/protectorate/aegisaltpistol/beamend.png;0;0?replace;a355c0a5=00010000;a355c07b=2b010000;ffffffa5=00012b00;ffffff7b=2b012b00?scale=43;43?crop;0;0;43;43";

pub fn generate(img: image::Image) -> String {
    let hat_color_table = image::to_color_table(
        img,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );

    let hat_template_frames_vec = HAT_TEMPLATE_FRAMES
        .iter()
        .map(|row| row.iter().map(|&s| s.to_string()).collect())
        .collect();

    let template_hat_color_table = template::create(43, 43, hat_template_frames_vec);
    let diffrent = image::diffrent(template_hat_color_table, hat_color_table);

    let res = CHEST_DIRECTIVES.to_string() + &directives::to_replace(diffrent, false);
    res
}

pub fn lua_generate(_: &mlua::Lua, userdata: mlua::AnyUserData) -> mlua::Result<String> {
    let img = image::read_image(userdata);
    Ok(generate(img))
}
