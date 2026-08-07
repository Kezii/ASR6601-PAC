#[doc = "Register `DAR_H` reader"]
pub type R = crate::R<DarHSpec>;
#[doc = "Register `DAR_H` writer"]
pub type W = crate::W<DarHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dar_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DarHSpec;
impl crate::RegisterSpec for DarHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar_h::R`](R) reader structure"]
impl crate::Readable for DarHSpec {}
#[doc = "`write(|w| ..)` method takes [`dar_h::W`](W) writer structure"]
impl crate::Writable for DarHSpec {
    type Safety = crate::Unsafe;
}
