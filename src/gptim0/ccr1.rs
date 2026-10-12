#[doc = "Register `CCR1` reader"]
pub type R = crate::R<Ccr1Spec>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<Ccr1Spec>;
#[doc = "Field `CCR1` reader - capture compare 1 value, 16-bit"]
pub type Ccr1R = crate::FieldReader<u16>;
#[doc = "Field `CCR1` writer - capture compare 1 value, 16-bit"]
pub type Ccr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - capture compare 1 value, 16-bit"]
    #[inline(always)]
    pub fn ccr1(&self) -> Ccr1R {
        Ccr1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - capture compare 1 value, 16-bit"]
    #[inline(always)]
    pub fn ccr1(&mut self) -> Ccr1W<'_, Ccr1Spec> {
        Ccr1W::new(self, 0)
    }
}
#[doc = "GPTIM capture compare 1 value, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr1Spec;
impl crate::RegisterSpec for Ccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for Ccr1Spec {
    type Safety = crate::Unsafe;
}
