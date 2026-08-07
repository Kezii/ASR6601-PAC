#[doc = "Register `RAW_TFR_L` reader"]
pub type R = crate::R<RawTfrLSpec>;
#[doc = "Register `RAW_TFR_L` writer"]
pub type W = crate::W<RawTfrLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_tfr_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_tfr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawTfrLSpec;
impl crate::RegisterSpec for RawTfrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_tfr_l::R`](R) reader structure"]
impl crate::Readable for RawTfrLSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_tfr_l::W`](W) writer structure"]
impl crate::Writable for RawTfrLSpec {
    type Safety = crate::Unsafe;
}
