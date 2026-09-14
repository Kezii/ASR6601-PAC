#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Field `PS` reader - pin\\[15:0\\] pull-up/pull-down selection"]
pub type PsR = crate::FieldReader<u16>;
#[doc = "Field `PS` writer - pin\\[15:0\\] pull-up/pull-down selection"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] pull-up/pull-down selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] pull-up/pull-down selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, PsrSpec> {
        PsW::new(self, 0)
    }
}
#[doc = "pull select register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
}
