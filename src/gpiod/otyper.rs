#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OtyperSpec>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OtyperSpec>;
#[doc = "Field `OTYPE` reader - pin\\[15:0\\] output type control"]
pub type OtypeR = crate::FieldReader<u16>;
#[doc = "Field `OTYPE` writer - pin\\[15:0\\] output type control"]
pub type OtypeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output type control"]
    #[inline(always)]
    pub fn otype(&self) -> OtypeR {
        OtypeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output type control"]
    #[inline(always)]
    pub fn otype(&mut self) -> OtypeW<'_, OtyperSpec> {
        OtypeW::new(self, 0)
    }
}
#[doc = "output type register\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtyperSpec;
impl crate::RegisterSpec for OtyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OtyperSpec {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OtyperSpec {
    type Safety = crate::Unsafe;
}
