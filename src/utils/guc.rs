use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};

pub struct RandomGUC {
    pub seed: GucSetting<i32>,
    pub min_integer: GucSetting<i32>,
    pub max_integer: GucSetting<i32>,
    pub min_text_length: GucSetting<i32>,
    pub max_text_length: GucSetting<i32>,
    pub array_length: GucSetting<i32>,
    pub float_scale: GucSetting<i32>,
}

impl RandomGUC {
    pub const fn new() -> Self {
        Self {
            min_integer: GucSetting::<i32>::new(-10000),
            max_integer: GucSetting::<i32>::new(10000),
            min_text_length: GucSetting::<i32>::new(30000),
            max_text_length: GucSetting::<i32>::new(50000),
            array_length: GucSetting::<i32>::new(1024),
            float_scale: GucSetting::<i32>::new(1),
            seed: GucSetting::<i32>::new(0),
        }
    }

    pub fn init(&self) {
        GucRegistry::define_int_guc(
            "random.min_int",
            "",
            "",
            &self.min_integer,
            i32::MIN,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.max_int",
            "",
            "",
            &self.max_integer,
            i32::MIN,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.min_text_length",
            "",
            "",
            &self.min_text_length,
            3,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.max_text_length",
            "",
            "",
            &self.max_text_length,
            3,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.array_length",
            "",
            "",
            &self.array_length,
            1,
            16384,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.float_scale",
            "",
            "",
            &self.float_scale,
            1,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.seed",
            "",
            "",
            &self.seed,
            i32::MIN,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );
    }
}

pub static PARADE_GUC: RandomGUC = RandomGUC::new();
