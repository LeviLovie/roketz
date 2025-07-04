use anyhow::{Context, Result};
use fmod::{
    Utf8CStr,
    studio::{Bank, System},
};
use std::collections::HashMap;

pub struct SoundEngine {
    system: System,
    bank: Bank,
    instances: HashMap<String, fmod::studio::EventInstance>,
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
            let _ = Self::load_bank(&system, add).context("Failed to load strings bank")?;
        }
        let bank = Self::load_bank(&system, bank).context("Failed to load master bank")?;

        Ok(SoundEngine {
            system,
            bank,
            instances: HashMap::new(),
        })
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

    pub fn update(&mut self) -> Result<()> {
        self.system
            .update()
            .context("Failed to update FMOD system")?;
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

    pub fn play_looping(&mut self, event_path: &str) -> Result<()> {
        if self.instances.contains_key(event_path) {
            return Ok(());
        }

        let cstring = std::ffi::CString::new(event_path)
            .context("Failed to create CString from event path")?;
        let fmod_path =
            Utf8CStr::from_cstr(&cstring).context("Failed to convert CString to Utf8CStr")?;

        let description = self
            .system
            .get_event(fmod_path)
            .context(format!("Failed to get event description: {event_path}"))?;
        let instance = description
            .create_instance()
            .context("Failed to create event instance")?;
        instance.start()?;
        self.instances.insert(event_path.to_string(), instance);
        Ok(())
    }

    pub fn stop_looping(&mut self, event_path: &str) -> Result<()> {
        if let Some(instance) = self.instances.remove(event_path) {
            instance
                .stop(fmod::studio::StopMode::AllowFadeout)
                .context("Failed to stop event instance")?;
        }
        Ok(())
    }

    pub fn set_parameter(&mut self, event_path: &str, param: &str, value: f32) -> Result<()> {
        if let Some(instance) = self.instances.get_mut(event_path) {
            let cstring = std::ffi::CString::new(param)
                .context("Failed to create CString from parameter name")?;
            let param_name: &Utf8CStr = Utf8CStr::from_cstr(cstring.as_c_str())
                .context("Failed to convert CString to Utf8CStr")?;
            instance.set_parameter_by_name(param_name, value, true)?;
        }
        Ok(())
    }

    fn load_bank(system: &System, path: &str) -> Result<Bank> {
        let cstring = std::ffi::CString::new(path).context("Failed to create CString")?;
        let bank_path: &Utf8CStr = Utf8CStr::from_cstr(cstring.as_c_str())
            .context("Failed to convert CString to Utf8CStr")?;
        let bank = system
            .load_bank_file(bank_path, fmod::studio::LoadBankFlags::NORMAL)
            .context("Failed to load bank file")?;
        Ok(bank)
    }
}

impl Drop for SoundEngine {
    fn drop(&mut self) {
        if let Err(e) = unsafe { self.system.release() } {
            eprintln!("Failed to release FMOD system: {e}");
        }
    }
}
