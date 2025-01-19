use crate::directives;
use crate::utils::image::{self, Image};

use crate::template::create;

const HAT_TEMPLATE_FRAMES: [[&str; 1]; 1] = [["01"]];

const CHEST_DIRECTIVES: &str = "?setcolor=fff?replace;fff0=fff?crop;0;0;2;2?blendmult=/items/active/weapons/protectorate/aegisaltpistol/beamend.png;0;0?replace;a355c0a5=00010000;a355c07b=2b010000;ffffffa5=00012b00;ffffff7b=2b012b00?scale=43;43?crop;0;0;43;43";

pub fn generate_hat(img: Image) -> String {
    let color_hat = image::to_color_table(
        &img,
        image::ImageParseOptions {
            skip_transparent: true,
        },
    );
    let template_hat = create(
        43,
        43,
        HAT_TEMPLATE_FRAMES
            .iter()
            .map(|x| x.iter().map(|x| x.to_string()).collect())
            .collect(),
    );
    let diffrent = image::diffrent(template_hat, color_hat);

    let res = CHEST_DIRECTIVES.to_string() + &directives::to_replace(diffrent, false);
    res
}
