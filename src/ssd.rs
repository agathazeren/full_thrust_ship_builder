use yew::prelude::*;
use crate::ships::{self,Ship,System};

#[derive(Clone)]
pub struct Icon{
    system:System
}

#[derive(Clone,PartialEq)]
pub struct IconProps{
    pub system:System
}

impl Default for IconProps{
    fn default()->Self{
        IconProps{
            system:System::cannon("Beam",1,crate::ships::Arcs::all())
        }
    }
}

impl Component for Icon{
    type Properties = IconProps;
    type Message = ();

    fn create(props:Self::Properties,_:ComponentLink<Self>)->Self{
        Icon{system:props.system}
    }

    fn update(&mut self,_:Self::Message)->ShouldRender{
        false
    }
}


impl Renderable<Self> for Icon{
    fn view(&self)->Html<Self>{
        html!{
            <div class="icon-temp"></div>
        }
    }
}



pub struct SSD{
    ship:Ship
}

#[derive(Clone,PartialEq)]
pub struct SSDProps{
    pub ship:Ship
}

impl Default for SSDProps{
    fn default()->Self{
        SSDProps{
            ship: ships::TEST_SHIP.clone()
        }
    }
}

impl Component for SSD{
    type Properties = SSDProps;
    type Message = ();

    fn create(props:SSDProps,_:ComponentLink<Self>)->Self{
        SSD{
            ship:props.ship
        }
    }

    fn update(&mut self,_:())->ShouldRender{
        false
    }
}


impl Renderable<SSD> for SSD{
    fn view(&self)->Html<Self>{
        html!{
            <div class="ssd-box">
                <span class="ssd-name">{self.ship.name.to_owned()}</span>
            </div>
        }
    }
}




















