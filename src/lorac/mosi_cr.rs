#[doc = "Register `MOSI_CR` reader"]
pub type R = crate::R<MosiCrSpec>;
#[doc = "Register `MOSI_CR` writer"]
pub type W = crate::W<MosiCrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "mosi control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MosiCrSpec;
impl crate::RegisterSpec for MosiCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mosi_cr::R`](R) reader structure"]
impl crate::Readable for MosiCrSpec {}
#[doc = "`write(|w| ..)` method takes [`mosi_cr::W`](W) writer structure"]
impl crate::Writable for MosiCrSpec {
    type Safety = crate::Unsafe;
}
