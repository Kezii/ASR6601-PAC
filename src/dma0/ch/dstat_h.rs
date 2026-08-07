#[doc = "Register `DSTAT_H` reader"]
pub type R = crate::R<DstatHSpec>;
#[doc = "Register `DSTAT_H` writer"]
pub type W = crate::W<DstatHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstat_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstatHSpec;
impl crate::RegisterSpec for DstatHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstat_h::R`](R) reader structure"]
impl crate::Readable for DstatHSpec {}
#[doc = "`write(|w| ..)` method takes [`dstat_h::W`](W) writer structure"]
impl crate::Writable for DstatHSpec {
    type Safety = crate::Unsafe;
}
