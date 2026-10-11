#[doc = "Register `CCR0` reader"]
pub type R = crate::R<Ccr0Spec>;
#[doc = "Register `CCR0` writer"]
pub type W = crate::W<Ccr0Spec>;
#[doc = "Field `CCR0` reader - capture compare 0 value, 16-bit"]
pub type Ccr0R = crate::FieldReader<u16>;
#[doc = "Field `CCR0` writer - capture compare 0 value, 16-bit"]
pub type Ccr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - capture compare 0 value, 16-bit"]
    #[inline(always)]
    pub fn ccr0(&self) -> Ccr0R {
        Ccr0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - capture compare 0 value, 16-bit"]
    #[inline(always)]
    pub fn ccr0(&mut self) -> Ccr0W<'_, Ccr0Spec> {
        Ccr0W::new(self, 0)
    }
}
#[doc = "GPTIM capture compare 0 value, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr0Spec;
impl crate::RegisterSpec for Ccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr0::R`](R) reader structure"]
impl crate::Readable for Ccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr0::W`](W) writer structure"]
impl crate::Writable for Ccr0Spec {
    type Safety = crate::Unsafe;
}
