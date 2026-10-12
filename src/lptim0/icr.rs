#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CMPMCF` writer - Clear compare match flag"]
pub type CmpmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Clear autoreload match flag"]
pub type ArrmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - Clear external trigger flag"]
pub type ExttrigcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKCF` writer - Clear compare register update ok flag"]
pub type CmpokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Clear autoreload register update ok flag"]
pub type ArrokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Clear direction change to up flag"]
pub type UpcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Clear direction change to down flag"]
pub type DowncfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear compare match flag"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CmpmcfW<'_, IcrSpec> {
        CmpmcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear autoreload match flag"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ArrmcfW<'_, IcrSpec> {
        ArrmcfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear external trigger flag"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> ExttrigcfW<'_, IcrSpec> {
        ExttrigcfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear compare register update ok flag"]
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CmpokcfW<'_, IcrSpec> {
        CmpokcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear autoreload register update ok flag"]
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ArrokcfW<'_, IcrSpec> {
        ArrokcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear direction change to up flag"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UpcfW<'_, IcrSpec> {
        UpcfW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear direction change to down flag"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DowncfW<'_, IcrSpec> {
        DowncfW::new(self, 6)
    }
}
#[doc = "LPTIM flag clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
