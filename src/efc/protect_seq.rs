#[doc = "Register `PROTECT_SEQ` writer"]
pub type W = crate::W<ProtectSeqSpec>;
#[doc = "Field `PROTECT_SEQ` writer - Protection sequence"]
pub type ProtectSeqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Protection sequence"]
    #[inline(always)]
    pub fn protect_seq(&mut self) -> ProtectSeqW<'_, ProtectSeqSpec> {
        ProtectSeqW::new(self, 0)
    }
}
#[doc = "protect sequence register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect_seq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtectSeqSpec;
impl crate::RegisterSpec for ProtectSeqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`protect_seq::W`](W) writer structure"]
impl crate::Writable for ProtectSeqSpec {
    type Safety = crate::Unsafe;
}
