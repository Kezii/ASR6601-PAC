#[doc = "Register `BSR` writer"]
pub type W = crate::W<BsrSpec>;
#[doc = "Field `BSR` writer - pin\\[15:0\\] output data set"]
pub type BsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BR` writer - pin\\[15:0\\] output data clear"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output data set"]
    #[inline(always)]
    pub fn bsr(&mut self) -> BsrW<'_, BsrSpec> {
        BsrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - pin\\[15:0\\] output data clear"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, BsrSpec> {
        BrW::new(self, 16)
    }
}
#[doc = "bit set register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrSpec;
impl crate::RegisterSpec for BsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsr::W`](W) writer structure"]
impl crate::Writable for BsrSpec {
    type Safety = crate::Unsafe;
}
