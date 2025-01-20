use crate::utils::{directives, image, template};

const BACK_TEMPLATE_FRAMES: [[&str; 9]; 7] = [
    ["", "a1", "a2", "a3", "a4", "a5", "a6", "", "a7"],
    ["", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"],
    ["", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8"],
    ["", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8"],
    ["", "", "", "", "", "", "", "", ""],
    ["", "e1", "", "", "e2", "e3", "e4", "e5", ""],
    ["", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8"],
];

const BACK_DIRECTIVES: &str = "?scale=0.4?scale=0.7?scale=0.84?crop;4;2;5;3?replace;aa836459=ffa1ff00;bb885e4e=ffa2ff00;cb926431=ffa3ff00;cb95693b=ffa4ff00;cf91601c=ffa5ff00;ce966b4b=ffa6ff00;e6c2a50c=ffa7ff00;cc93662e=ffb1ff00;cc8c5921=ffb2ff00;c1895c3d=ffb3ff00;bf885c41=ffb4ff00;cb8e5d2d=ffb5ff00;c7895728=ffb6ff00;ac7c5558=ffb7ff00;b8a99d5d=ffb8ff00;d796610f=ffc1ff00;dc955b0a=ffc2ff00;de965b06=ffc3ff00;c3885730=ffc4ff00;d9945b0d=ffc5ff00;dc955b08=ffc6ff00;da945b0a=ffc7ff00;dfbca11e=ffc8ff00;ce8e5b23=ffd1ff00;dc945b06=ffd2ff00;cf8f5b23=ffd3ff00;9d74526f=ffd4ff00;d28f5916=ffd5ff00;de965b02=ffd6ff00;e0975c00=ffd7ff00;ecc3a200=ffd8ff00;d08f5b1e=ffe1ff00;da945b08=ffe2ff00;cd905f2f=ffe3ff00;d8955e14=ffe4ff00;cc8d5920=ffe5ff00;59504932=fff1ff00;655c5509=fff2ff00;7369631b=fff3ff00;756c665a=fff4ff00;62574f32=fff5ff00;877d7782=fff6ff00;63595277=fff7ff00;a19d9959=fff8ff00?scalenearest=1;2?blendmult=/monsters/boss/apeboss/apeboss.png;1263;394?scalenearest=2;1?blendmult=/dungeons/other/wreck/key.png;755;29?multiply=2eff2e?scale=47?crop;1;1;44;44";

pub fn generate(back_image: image::Image) -> String {
    let back_color_table = image::to_color_table(
        back_image,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );

    let back_template_frames_vec: Vec<Vec<String>> = BACK_TEMPLATE_FRAMES
        .iter()
        .map(|row| row.iter().map(|&s| s.to_string()).collect())
        .collect();
    let template_back_color_table = template::create(43, 43, back_template_frames_vec);

    let diffrent = image::diffrent(template_back_color_table, back_color_table);

    BACK_DIRECTIVES.to_string() + &directives::to_replace(diffrent, false)
}

pub fn lua_generate(_: &mlua::Lua, userdata: mlua::AnyUserData) -> mlua::Result<String> {
    let img = image::read_image(userdata);
    Ok(generate(img))
}
