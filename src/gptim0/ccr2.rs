#[doc = "Register `CCR2` reader"]
pub type R = crate::R<Ccr2Spec>;
#[doc = "Register `CCR2` writer"]
pub type W = crate::W<Ccr2Spec>;
#[doc = "Field `CCR2` reader - capture compare 2 value, 16-bit"]
pub type Ccr2R = crate::FieldReader<u16>;
#[doc = "Field `CCR2` writer - capture compare 2 value, 16-bit"]
pub type Ccr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - capture compare 2 value, 16-bit"]
    #[inline(always)]
    pub fn ccr2(&self) -> Ccr2R {
        Ccr2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - capture compare 2 value, 16-bit"]
    #[inline(always)]
    pub fn ccr2(&mut self) -> Ccr2W<'_, Ccr2Spec> {
        Ccr2W::new(self, 0)
    }
}
#[doc = "GPTIM capture compare 2 value, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr2Spec;
impl crate::RegisterSpec for Ccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2::R`](R) reader structure"]
impl crate::Readable for Ccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr2::W`](W) writer structure"]
impl crate::Writable for Ccr2Spec {
    type Safety = crate::Unsafe;
}
