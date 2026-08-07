#[doc = "Register `INT_SR` reader"]
pub type R = crate::R<IntSrSpec>;
#[doc = "Register `INT_SR` writer"]
pub type W = crate::W<IntSrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSrSpec;
impl crate::RegisterSpec for IntSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_sr::R`](R) reader structure"]
impl crate::Readable for IntSrSpec {}
#[doc = "`write(|w| ..)` method takes [`int_sr::W`](W) writer structure"]
impl crate::Writable for IntSrSpec {
    type Safety = crate::Unsafe;
}
