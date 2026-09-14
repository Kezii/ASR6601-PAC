#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `SYSCFG_DBG_STANDBY` reader - allow debug connection in standby mode"]
pub type SyscfgDbgStandbyR = crate::BitReader;
#[doc = "Field `SYSCFG_DBG_STANDBY` writer - allow debug connection in standby mode"]
pub type SyscfgDbgStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_DBG_STOP` reader - allow debug connection in stop mode"]
pub type SyscfgDbgStopR = crate::BitReader;
#[doc = "Field `SYSCFG_DBG_STOP` writer - allow debug connection in stop mode"]
pub type SyscfgDbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - allow debug connection in standby mode"]
    #[inline(always)]
    pub fn syscfg_dbg_standby(&self) -> SyscfgDbgStandbyR {
        SyscfgDbgStandbyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - allow debug connection in stop mode"]
    #[inline(always)]
    pub fn syscfg_dbg_stop(&self) -> SyscfgDbgStopR {
        SyscfgDbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - allow debug connection in standby mode"]
    #[inline(always)]
    pub fn syscfg_dbg_standby(&mut self) -> SyscfgDbgStandbyW<'_, Cr3Spec> {
        SyscfgDbgStandbyW::new(self, 0)
    }
    #[doc = "Bit 1 - allow debug connection in stop mode"]
    #[inline(always)]
    pub fn syscfg_dbg_stop(&mut self) -> SyscfgDbgStopW<'_, Cr3Spec> {
        SyscfgDbgStopW::new(self, 1)
    }
}
#[doc = "control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
