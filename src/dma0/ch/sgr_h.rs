#[doc = "Register `SGR_H` reader"]
pub type R = crate::R<SgrHSpec>;
#[doc = "Register `SGR_H` writer"]
pub type W = crate::W<SgrHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgrHSpec;
impl crate::RegisterSpec for SgrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr_h::R`](R) reader structure"]
impl crate::Readable for SgrHSpec {}
#[doc = "`write(|w| ..)` method takes [`sgr_h::W`](W) writer structure"]
impl crate::Writable for SgrHSpec {
    type Safety = crate::Unsafe;
}
