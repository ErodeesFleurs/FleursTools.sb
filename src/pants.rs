use crate::directives;
use crate::image;

use crate::template::create;

const PANTS_TEMPLATE_FRAMES: [[&str; 9]; 6] = [
    ["", "a1", "a2", "a3", "a4", "a5", "a6", "", "a7"],
    ["", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"],
    ["", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8"],
    ["", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8"],
    ["", "", "", "", "", "", "", "", ""],
    ["", "e1", "", "", "e2", "e3", "e4", "e5", ""],
];

const PANTS_DIRECTIVES: &str = "?scale=0.4?scale=0.7?crop;6;2;7;3?replace;a0b03e=ffa1ff00;7e9b35=ffa2ff00;45483887=ffa3ff00;698635ef=ffa4ff00;405e2fe4=ffa5ff00;51362dc0=ffa6ff00;59353091=ffa7ff00;7c9036=ffb1ff00;6d702af4=ffb2ff00;91a638=ffb3ff00;748e37=ffb4ff00;746f2c=ffb5ff00;7a8a31=ffb6ff00;608333=ffb7ff00;8f953a=ffb8ff00;736f2f=ffc1ff00;41373b5d=ffc2ff00;515f38ab=ffc3ff00;788e35=ffc4ff00;6f602f=ffc5ff00;273430ab=ffc6ff00;617e34=ffc7ff00;829935=ffc8ff00;2d173b2e=ffd1ff00;2b243668=ffd2ff00;725830c0=ffd3ff00;7b4d31ca=ffd4ff00;663c2dab=ffd5ff00;5735376d=ffd6ff00;5d3a3877=ffd7ff00;52403496=ffd8ff00;55662dd5=ffe1ff00;8088318c=ffe2ff00;778c34=ffe3ff00;8c7835a1=ffe4ff00;668c3487=ffe5ff00?scalenearest=1;2?blendmult=/monsters/boss/apeboss/apeboss.png;1263;394?scalenearest=2;1?blendmult=/dungeons/other/wreck/key.png;755;29?multiply=2eff2e?scale=47?crop;1;1;44;44";
const PANTS_HIDE_BODY_DIRECTIVES: &str = "?scale=0.4?scale=0.7?scale=0.85?crop;6;1;7;2?replace;45572af4=ffa1ff00;4e6530f8=ffa2ff00;4f6631f7=ffa3ff00;445729f7=ffa4ff00;4f6531f5=ffa5ff00;487035fc=ffa6ff00;445e2df8=ffa7ff00;556733f4=ffb1ff00;4a4122f6=ffb2ff00;425929f0=ffb3ff00;3e2f1cb7=ffb4ff00;664d39bb=ffb5ff00;425b2df2=ffb6ff00;54201e7f=ffb7ff00;56322780=ffb8ff00;675831bb=ffc1ff00;5916192a=ffc2ff00;463c20c0=ffc3ff00;36311bb7=ffc4ff00;313c1ebb=ffc5ff00;561b1a29=ffc6ff00;38642bf4=ffc7ff00;60493180=ffc8ff00;415128bb=ffd1ff00;47562ab9=ffd2ff00;4939207f=ffd3ff00;4f432681=ffd4ff00;5a1e1b25=ffd5ff00;5a1e1c25=ffd6ff00;571a1a25=ffd7ff00;62212413=ffd8ff00;29452424=ffe1ff00;96918a25=ffe2ff00;4567347f=ffe3ff00;5a755681=ffe4ff00;427235f3=ffe5ff00?scalenearest=1;2?blendmult=/monsters/boss/apeboss/apeboss.png;1263;394?scalenearest=2;1?blendmult=/dungeons/other/wreck/key.png;755;29?multiply=2eff2e?scale=47?crop;1;1;44;44";

pub fn generate_pants(pants_path: String, hide_body: bool) -> String {
    let img = image::Image::new(&pants_path);

    let color_pants = image::to_color_table(
        &img,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );

    let template_pants = create(
        43,
        43,
        PANTS_TEMPLATE_FRAMES
            .iter()
            .map(|row| row.iter().map(|&s| s.to_string()).collect())
            .collect(),
    );
    let diffrent = image::diffrent(template_pants, color_pants);

    let res: String;
    if hide_body {
        res = PANTS_HIDE_BODY_DIRECTIVES.to_string() + &directives::to_replace(diffrent, false);
    } else {
        res = PANTS_DIRECTIVES.to_string() + &directives::to_replace(diffrent, false);
    }
    res
}
