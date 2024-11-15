struct SubwaySurfersGameplay {
    engaging: bool,
    length: u64,
    rizzler: bool,
}

struct ExtraVerifiedMisinformation(bool, bool, bool);

struct ThisIsntActuallyUseless;

impl SubwaySurfersGameplay {
    fn make_engaging_gameplay(length: u64, rizzler: bool) -> Self {
        SubwaySurfersGameplay {
            engaging: true,
            length,
            rizzler,
        }
    }

    fn length_in_mins(&self, length: u64) -> u64 {
        length / 60
    }
}

impl ThisIsntActuallyUseless {
    fn be_annoying() {
        println!("Hey Apple!");
    }
}

impl ThisIsntActuallyUseless {
    fn be_annoying_2() {
        println!("Skibidi Toilet");
    }
}

fn structs() {
    let mut gameplay = SubwaySurfersGameplay {
        engaging: true,
        length: 31536000,
        rizzler: true,
    };

    gameplay.length += 1;

    let unengaging_gameplay = SubwaySurfersGameplay {
        engaging: false,
        ..SubwaySurfersGameplay::make_engaging_gameplay(10, false)
    };

    let true_info = ExtraVerifiedMisinformation(true, true, true);

    let _ = ThisIsntActuallyUseless;
}
