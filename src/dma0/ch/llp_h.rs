#[doc = "Register `LLP_H` reader"]
pub type R = crate::R<LlpHSpec>;
#[doc = "Register `LLP_H` writer"]
pub type W = crate::W<LlpHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`llp_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LlpHSpec;
impl crate::RegisterSpec for LlpHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp_h::R`](R) reader structure"]
impl crate::Readable for LlpHSpec {}
#[doc = "`write(|w| ..)` method takes [`llp_h::W`](W) writer structure"]
impl crate::Writable for LlpHSpec {
    type Safety = crate::Unsafe;
}
