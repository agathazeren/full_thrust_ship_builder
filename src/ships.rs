pub struct Ship{
    systems: Vec<System>,
    thrust_rating:i8,
    advanced_drive:bool,
    hull_rows:i8,
    hull_count:i16,
    armor:Vec<ArmorRow>,    
}

#[derive(PartialEq,Clone)]
pub struct System{
    pub name: String,

    pub design: SystemDesign
}

#[derive(PartialEq,Clone)]
pub enum SystemDesign{
    Cannon(i8,Arcs),Identity
}

impl System{
    pub fn identity(name:&str)->System{
        System{name:name.to_string(),design:SystemDesign::Identity}
    }
    
    pub fn cannon(name:&str,level:i8,arcs:Arcs)->System{
        System{name:name.to_string(),design:SystemDesign::Cannon(level,arcs)}
    }
}

#[derive(PartialEq,Copy,Clone)]
enum Facing{
    Starboard,Center,Port
}

#[derive(PartialEq,Copy,Clone)]
enum Forwardness{
    Fore,Aft
}

#[derive(PartialEq,Copy,Clone)]
struct Arc{
    pub facing: Facing,
    pub fowardness: Forwardness
}

#[derive(PartialEq,Clone)]
pub struct Arcs{
    fs:bool,fc:bool,fp:bool,r#as:bool,ac:bool,ap:bool
}

impl Arcs{
    pub fn all()->Arcs{
        Arcs{fs:true,fc:true,fp:true,r#as:true,ac:true,ap:true}
    }
    pub fn forward3()->Arcs{
        Arcs{fs:true,fc:true,fp:true,r#as:false,ac:false,ap:false}
    }


    fn get(&self,arc:Arc)->bool{
        use Facing::*;
        use Forwardness::*;
        match (arc.fowardness,arc.facing){
            (Fore,Starboard)=>self.fs,
            (Fore,Center)=>self.fc,
            (Fore,Port)=>self.fp,
            (Aft,Starboard)=>self.r#as,
            (Aft,Center)=>self.ac,
            (Aft,Port)=>self.ap
        }
    }

    pub fn count(&self)->u8{
        (if self.fs {1} else {0})
            +(if self.fc {1} else {0})
            +(if self.fp {1} else {0})
            +(if self.r#as {1} else {0})
            +(if self.ac {1} else {0})
            +(if self.ap {1} else {0})
    }
}



impl std::ops::BitOr<Arcs> for Arcs{
  type Output = Arcs;
  
  fn bitor(self,rhs:Arcs)->Arcs{
      Arcs{
          fs: self.fs || rhs.fs,
          fc: self.fc || rhs.fc,
          fp: self.fp || rhs.fp,
          r#as: self.r#as || rhs.r#as,
          ac: self.ac || rhs.ac,
          ap: self.ap || rhs.ap,
      }
  }
}


struct ArmorRow{
    kind:ArmorKind,
    size:i8,
}

enum ArmorKind{
    Standard,Regenerative
}


fn main(){
    let ff = Ship{
        thrust_rating:6,
        advanced_drive:false,
        hull_count: 5,
        hull_rows:4,
        armor: vec![],
        systems: vec![
            System::identity("PDS"),
            System::identity("FCS"),
            System::cannon("Beam",1,Arcs::all()),
            System::cannon("Beam",1,Arcs::all()),
            System::cannon("Beam",2,Arcs::forward3())
        ]
    };
}
