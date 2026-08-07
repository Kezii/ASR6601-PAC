#[doc = "Register `MASK_TFR_H` reader"]
pub type R = crate::R<MaskTfrHSpec>;
#[doc = "Register `MASK_TFR_H` writer"]
pub type W = crate::W<MaskTfrHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_tfr_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_tfr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskTfrHSpec;
impl crate::RegisterSpec for MaskTfrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_tfr_h::R`](R) reader structure"]
impl crate::Readable for MaskTfrHSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_tfr_h::W`](W) writer structure"]
impl crate::Writable for MaskTfrHSpec {
    type Safety = crate::Unsafe;
}
