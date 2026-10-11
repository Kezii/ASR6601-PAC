#[doc = "Register `SWTRIGR` writer"]
pub type W = crate::W<SwtrigrSpec>;
#[doc = "Field `SWTRIG` writer - Software trigger, write 1 to trigger conversion"]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software trigger, write 1 to trigger conversion"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<'_, SwtrigrSpec> {
        SwtrigW::new(self, 0)
    }
}
#[doc = "software trigger register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigrSpec;
impl crate::RegisterSpec for SwtrigrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure"]
impl crate::Writable for SwtrigrSpec {
    type Safety = crate::Unsafe;
}
