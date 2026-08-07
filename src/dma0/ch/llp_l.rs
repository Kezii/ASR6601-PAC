#[doc = "Register `LLP_L` reader"]
pub type R = crate::R<LlpLSpec>;
#[doc = "Register `LLP_L` writer"]
pub type W = crate::W<LlpLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`llp_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LlpLSpec;
impl crate::RegisterSpec for LlpLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp_l::R`](R) reader structure"]
impl crate::Readable for LlpLSpec {}
#[doc = "`write(|w| ..)` method takes [`llp_l::W`](W) writer structure"]
impl crate::Writable for LlpLSpec {
    type Safety = crate::Unsafe;
}
