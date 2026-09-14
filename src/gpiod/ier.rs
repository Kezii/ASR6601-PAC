#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `IE` reader - pin\\[15:0\\] input enable"]
pub type IeR = crate::FieldReader<u16>;
#[doc = "Field `IE` writer - pin\\[15:0\\] input enable"]
pub type IeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] input enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] input enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, IerSpec> {
        IeW::new(self, 0)
    }
}
#[doc = "input enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
