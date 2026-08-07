#[doc = "Register `DMAR` reader"]
pub type R = crate::R<DmarSpec>;
#[doc = "Register `DMAR` writer"]
pub type W = crate::W<DmarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIMER DMA address for full transfer register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmar::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarSpec;
impl crate::RegisterSpec for DmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmar::R`](R) reader structure"]
impl crate::Readable for DmarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmar::W`](W) writer structure"]
impl crate::Writable for DmarSpec {
    type Safety = crate::Unsafe;
}
