#[doc = "Register `OER` reader"]
pub type R = crate::R<OerSpec>;
#[doc = "Register `OER` writer"]
pub type W = crate::W<OerSpec>;
#[doc = "Field `OEN` reader - pin\\[15:0\\] output enable"]
pub type OenR = crate::FieldReader<u16>;
#[doc = "Field `OEN` writer - pin\\[15:0\\] output enable"]
pub type OenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output enable"]
    #[inline(always)]
    pub fn oen(&self) -> OenR {
        OenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output enable"]
    #[inline(always)]
    pub fn oen(&mut self) -> OenW<'_, OerSpec> {
        OenW::new(self, 0)
    }
}
#[doc = "output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`oer::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OerSpec;
impl crate::RegisterSpec for OerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oer::R`](R) reader structure"]
impl crate::Readable for OerSpec {}
#[doc = "`write(|w| ..)` method takes [`oer::W`](W) writer structure"]
impl crate::Writable for OerSpec {
    type Safety = crate::Unsafe;
}
