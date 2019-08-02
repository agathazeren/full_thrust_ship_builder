#![feature(never_type)]

use yew::prelude::*;



#[macro_use]
extern crate lazy_static;

mod ships;
mod ssd;
mod system_view;


struct ShipPane{
    ship:ships::Ship

}

#[derive(Clone,PartialEq)]
struct ShipPaneProps{
    ship:ships::Ship
}

impl Default for ShipPaneProps{
    fn default()->Self{
        ShipPaneProps{
            ship:ships::TEST_SHIP.clone()
        }
    }
}

impl Component for ShipPane{
    type Message = !;
    type Properties = ShipPaneProps;

    fn create(props:ShipPaneProps,_:ComponentLink<Self>)->Self{
        ShipPane{
            ship:props.ship
        }
    }

    fn update(&mut self,_:!)->ShouldRender{
        false
    }
}

impl Renderable<Self> for ShipPane{
    fn view(&self)->Html<Self>{
        use system_view::SystemList;
        use ssd::SSD;
        html!{
            <div id="ship-pane">
                <SSD ship=self.ship.clone()/>
                <SystemList systems=self.ship.systems.clone()/>
            </div>
        }
    }
}
                






fn main(){
    yew::start_app::<ShipPane>();
}
