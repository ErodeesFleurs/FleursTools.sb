use crate::directives;
use crate::image;

use crate::template::create;

const CHEST_TEMPLATE_FRAMES: [[&str; 2]; 6] = [
    ["", "f3"],
    ["f4", "f5"],
    ["", "f6"],
    ["", "f7"],
    ["", ""],
    ["", "f8"],
];

const FRONT_SLEEVE_TEMPLATE_FRAMES: [[&str; 9]; 7] = [
    ["", "a1", "a2", "a3", "a4", "a5", "", "", "a6"],
    ["", "", "b1", "b2", "b3", "b4", "b5", "", "b6"],
    ["", "", "c1", "c2", "c3", "c4", "c5", "", ""],
    ["", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8"],
    ["", "", "", "", "", "", "", "", ""],
    ["", "e1", "e2", "", "e3", "e4", "e5", "e6", "e7"],
    ["", "", "", "f1", "", "", "", "", "f2"],
];

const BACK_SLEEVE_TEMPLATE_FRAMES: [[&str; 9]; 7] = [
    ["", "60", "61", "62", "63", "64", "", "", "65"],
    ["", "", "66", "67", "68", "69", "6a", "", "6b"],
    ["", "", "6c", "6d", "6e", "6f", "70", "", ""],
    ["", "71", "72", "73", "74", "75", "76", "77", "78"],
    ["", "", "", "", "", "", "", "", ""],
    ["", "79", "7a", "", "7b", "7c", "7d", "7e", "7f"],
    ["", "", "", "80", "", "", "", "", "81"],
];

const CHEST_DIRECTIVES: &str = "?scale=0.4?scale=0.7?scale=0.85?scale=0.925?scale=0.9625?scale=0.8?scale=0.8?crop;3;3;4;4?replace;bdcc640b=ffa1ff;bbc9620f=ffa2ff;bdcd620a=ffa3ff;bccb620d=ffa4ff;bccb630d=ffa5ff;aac05e06=ffa6ff;bfc75d1d=ffb1ff;c8d25f11=ffb2ff;c0c85f1c=ffb3ff;cad65f0b=ffb4ff;cbd96008=ffb5ff;9ba75427=ffb6ff;b1a95b48=ffc1ff;bebe5b2f=ffc2ff;c8d16115=ffc3ff;c9d4600e=ffc4ff;c6ce6015=ffc5ff;bec55e1e=ffd1ff;cad5610c=ffd2ff;c9d5600d=ffd3ff;c6d05d11=ffd4ff;c7d25e10=ffd5ff;c9d16112=ffd6ff;c8d45e0d=ffd7ff;9eb05018=ffd8ff;b7b75f33=ffe1ff;bec25e25=ffe2ff;c4c95c1e=ffe3ff;b5b05840=ffe4ff;c2c85f1e=ffe5ff;bcb85f3b=ffe6ff;a0a9562c=ffe7ff;c8d55e0b=fff1ff;a3b9510b=fff2ff;40432222=ff60ff;a2ab501e=ff61ff;979a4d2f=ff62ff;a2ac501c=ff63ff;a0a9501f=ff64ff;9ead4709=ff65ff;c5cb651d=ff66ff;c5cd631a=ff67ff;c1c46524=ff68ff;bdc45c21=ff69ff;c0c46325=ff6aff;b6b46531=ff6bff;c3c7621e=ff6cff;c4ca621c=ff6dff;bdc45c20=ff6eff;c1c8601e=ff6fff;b8bd5b29=ff70ff;55522f23=ff71ff;b4b15f3b=ff72ff;b8be5b29=ff73ff;b3b25e39=ff74ff;a7a85642=ff75ff;a29f544d=ff76ff;a9ab553d=ff77ff;a0a1523a=ff78ff;53522c20=ff79ff;bcc45c20=ff7aff;b8be5b28=ff7bff;c2c7571f=ff7cff;c7cc651b=ff7dff;b9bd5f2c=ff7eff;bcc25f1b=ff7fff;bcc35b21=ff80ff;bfcc5807=ff81ff;2c221d19=fff3ff;9f853e19=fff4ff;8a8d4812=fff5ff;8c954a02=fff7ff;8c8f4a18=fff6ff;8f8f4f1c=fff8ff;46352a36=fff3ff;85703442=fff4ff;8c834737=fff5ff;8c954a05=fff7ff;988e4f3f=fff6ff;8e864d36=fff8ff?scalenearest=1;2?blendmult=/monsters/boss/apeboss/apeboss.png;1263;394?scalenearest=2;1?blendmult=/dungeons/other/wreck/key.png;755;29?multiply=2eff2e00?scale=47?crop;1;1;44;44";

pub fn generate_chest(
    chest_path: String,
    front_sleeve_path: String,
    back_sleeve_path: String,
) -> String {
    let chest_img = image::Image::new(&chest_path);
    let front_sleeve_img = image::Image::new(&front_sleeve_path);
    let back_sleeve_img = image::Image::new(&back_sleeve_path);

    let color_chest = image::to_color_table(
        &chest_img,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );

    let color_front_sleeve = image::to_color_table(
        &front_sleeve_img,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );

    let color_back_sleeve = image::to_color_table(
        &back_sleeve_img,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );

    let template_chest = create(
        43,
        43,
        CHEST_TEMPLATE_FRAMES
            .iter()
            .map(|row| row.iter().map(|&s| s.to_string()).collect())
            .collect(),
    );

    let template_front_sleeve = create(
        43,
        43,
        FRONT_SLEEVE_TEMPLATE_FRAMES
            .iter()
            .map(|row| row.iter().map(|&s| s.to_string()).collect())
            .collect(),
    );

    let template_back_sleeve = create(
        43,
        43,
        BACK_SLEEVE_TEMPLATE_FRAMES
            .iter()
            .map(|row| row.iter().map(|&s| s.to_string()).collect())
            .collect(),
    );

    let diffrent_chest = image::diffrent(template_chest, color_chest);
    let diffrent_front_sleeve = image::diffrent(template_front_sleeve, color_front_sleeve);
    let diffrent_back_sleeve = image::diffrent(template_back_sleeve, color_back_sleeve);

    let res = CHEST_DIRECTIVES.to_string()
        + &directives::to_replace(diffrent_chest, false)
        + &directives::to_replace(diffrent_front_sleeve, true)
        + &directives::to_replace(diffrent_back_sleeve, true);
    res
}
