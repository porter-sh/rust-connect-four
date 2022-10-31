use yew::{Children, Callback, Component, Html, html, Context, Properties};
use yew_router::prelude::*;
use crate::components::board::Disk;

use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: u8,
    pub disks: RefCell<[[Disk; 6]; 7]>
}

pub struct Column;

impl Component for Column {
    type Message = ();
    type Properties = ColumnProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button>{"col button"}</button>
        }
    }
}