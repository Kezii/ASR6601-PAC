#[doc = "Register `SGR_L` reader"]
pub type R = crate::R<SgrLSpec>;
#[doc = "Register `SGR_L` writer"]
pub type W = crate::W<SgrLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgrLSpec;
impl crate::RegisterSpec for SgrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr_l::R`](R) reader structure"]
impl crate::Readable for SgrLSpec {}
#[doc = "`write(|w| ..)` method takes [`sgr_l::W`](W) writer structure"]
impl crate::Writable for SgrLSpec {
    type Safety = crate::Unsafe;
}
