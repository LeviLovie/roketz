use anyhow::{Context, Result};
use fmod::{
    studio::{Bank, System},
    Utf8CStr,
};

pub struct SoundEngine {
    system: System,
    bank: Bank,
}

impl SoundEngine {
    pub fn new(bank: &str, adds: Vec<&str>) -> Result<Self> {
        let builder = unsafe { fmod::studio::SystemBuilder::new() }
            .context("Failed to create FMOD system builder")?;
        let system = builder
            .build(
                128,
                fmod::studio::InitFlags::NORMAL,
                fmod::InitFlags::NORMAL,
            )
            .context("Failed to build FMOD system")?;

        for add in adds {
            let _ = Self::load_bank(&system, &add).context("Failed to load strings bank")?;
        }
        let bank = Self::load_bank(&system, bank).context("Failed to load master bank")?;

        Ok(SoundEngine { system, bank })
    }

    pub fn list(&self) -> Result<()> {
        for event in self
            .bank
            .get_event_list()
            .context("Failed to get event list")?
        {
            println!("Event: {}", event.get_path()?);
        }
        Ok(())
    }

    pub fn play(&self, event_path: &str) -> Result<()> {
        let cstring = std::ffi::CString::new(event_path)
            .context("Failed to create CString from event path")?;
        let event_path: &Utf8CStr = Utf8CStr::from_cstr(cstring.as_c_str())
            .context("Failed to convert CString to Utf8CStr")?;
        let description = self
            .system
            .get_event(event_path)
            .context("Failed to get event description")?;
        let instance = description
            .create_instance()
            .context("Failed to create event instance")?;
        instance.start().context("Failed to start event instance")?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.system
            .update()
            .context("Failed to update FMOD system")?;
        Ok(())
    }

    fn load_bank(system: &System, path: &str) -> Result<Bank> {
        let cstring = std::ffi::CString::new(path).context("Failed to create CString")?;
        let bank_path: &Utf8CStr = Utf8CStr::from_cstr(cstring.as_c_str())
            .context("Failed to convert CString to Utf8CStr")?;
        let bank = system
            .load_bank_file(&bank_path, fmod::studio::LoadBankFlags::NORMAL)
            .context("Failed to load bank file")?;
        Ok(bank)
    }
}

impl Drop for SoundEngine {
    fn drop(&mut self) {
        if let Err(e) = unsafe { self.system.release() } {
            eprintln!("Failed to release FMOD system: {}", e);
        }
    }
}
