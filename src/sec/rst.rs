#[doc = "Register `RST` reader"]
pub type R = crate::R<RstSpec>;
#[doc = "Register `RST` writer"]
pub type W = crate::W<RstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "reset enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstSpec;
impl crate::RegisterSpec for RstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst::R`](R) reader structure"]
impl crate::Readable for RstSpec {}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RstSpec {
    type Safety = crate::Unsafe;
}
