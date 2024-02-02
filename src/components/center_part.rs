use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::ui::{title::Titles, title::TitleLevel, search::Search};
use crate::components::ui::{center_box::Box};

#[styled_component]
pub fn Centerpart() -> Html{
    
    let style = style!{

        float:left;
        width:45%;
        height:86%;
        padding:20px;
        border-radius:25px;
        margin:2%;
        margin-left:0px;
        margin-right:0px;
    }.unwrap();
    html!{
        <div class={style}>
            <Titles level={TitleLevel::One}>{"Today"}</Titles>
            <Search placeholder={"Add new task"}/>
            <Box/>

        </div>

    }
}
