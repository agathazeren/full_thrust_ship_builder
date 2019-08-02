
use yew::prelude::*;


mod ships;
mod ssd;

struct SystemView{
    system:ships::System,
}

#[derive(Clone,PartialEq)]
struct SystemViewProps{
    system:ships::System,
}

impl Default for SystemViewProps{
    fn default()->Self{
        SystemViewProps{
            system:ships::System::cannon("Beam",1,ships::Arcs::all())
        }
    }
}


impl Component for SystemView{
    type Message = ();
    type Properties = SystemViewProps;

    fn create(props:Self::Properties, _:ComponentLink<Self>)->Self{
        SystemView{system:props.system}
    }

    fn update(&mut self, _:Self::Message)->ShouldRender{
        false
    }

}

impl Renderable<SystemView> for SystemView{
    fn view(&self)-> Html<Self>{
        use ships::SystemDesign::*;
        let detail = match &self.system.design {
                    Identity => html!{},
                    Cannon(class, arcs) => html!{
                        <span> {format!("Class: {}, Arcs: {}",class,arcs.count())} </span>
                    }
                };


        
        html!{
            <div class="system-box">
                <ssd::Icon system={ships::System::identity("PDS")}/>
                <span>{self.system.name.clone()}</span><br/>
                {detail}
            </div>
        }
    }
}



#[derive(Clone)]
struct SystemList{
    systems:Vec<ships::System>
}

#[derive(Clone,PartialEq)]
struct SystemListProps{
    systems:Vec<ships::System>
}

impl Default for SystemListProps{
    fn default()->Self{
        SystemListProps{
            systems:vec![
                ships::System::cannon("Beam",3,ships::Arcs::forward3()),
                ships::System::identity("PDS"),
                ships::System::identity("FCS")
            ]
        }

    }
}



impl Component for SystemList{
    type Message = ();
    type Properties = SystemListProps;

    fn create(props:Self::Properties,_:ComponentLink<Self>)->Self{
        SystemList{systems:props.systems}
    }

    fn update(&mut self, _:Self::Message)->ShouldRender{
        false
    }

}

impl Renderable<SystemList> for SystemList{
    fn view(&self)->Html<SystemList>{
        html!{
            <div>
            {for self.systems.iter().map(|sys| html!{<SystemView system={sys}/>})}
            </div>
        }
    }
}



fn main(){
    yew::start_app::<SystemList>();
}
