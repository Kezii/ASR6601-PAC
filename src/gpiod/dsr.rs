#[doc = "Register `DSR` writer"]
pub type W = crate::W<DsrSpec>;
#[doc = "Field `DS` writer - pin\\[15:0\\] output drive strength"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output drive strength"]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<'_, DsrSpec> {
        DsW::new(self, 0)
    }
}
#[doc = "drive strength register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrSpec;
impl crate::RegisterSpec for DsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsr::W`](W) writer structure"]
impl crate::Writable for DsrSpec {
    type Safety = crate::Unsafe;
}
