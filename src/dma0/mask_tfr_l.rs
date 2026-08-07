#[doc = "Register `MASK_TFR_L` reader"]
pub type R = crate::R<MaskTfrLSpec>;
#[doc = "Register `MASK_TFR_L` writer"]
pub type W = crate::W<MaskTfrLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_tfr_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_tfr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskTfrLSpec;
impl crate::RegisterSpec for MaskTfrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_tfr_l::R`](R) reader structure"]
impl crate::Readable for MaskTfrLSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_tfr_l::W`](W) writer structure"]
impl crate::Writable for MaskTfrLSpec {
    type Safety = crate::Unsafe;
}
