#[doc = "Register `SEQR0` reader"]
pub type R = crate::R<Seqr0Spec>;
#[doc = "Register `SEQR0` writer"]
pub type W = crate::W<Seqr0Spec>;
#[doc = "Field `SEL0` reader - sequence 0 channel select"]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL0` writer - sequence 0 channel select"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL1` reader - sequence 1 channel select"]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL1` writer - sequence 1 channel select"]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL2` reader - sequence 2 channel select"]
pub type Sel2R = crate::FieldReader;
#[doc = "Field `SEL2` writer - sequence 2 channel select"]
pub type Sel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL3` reader - sequence 3 channel select"]
pub type Sel3R = crate::FieldReader;
#[doc = "Field `SEL3` writer - sequence 3 channel select"]
pub type Sel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL4` reader - sequence 4 channel select"]
pub type Sel4R = crate::FieldReader;
#[doc = "Field `SEL4` writer - sequence 4 channel select"]
pub type Sel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL5` reader - sequence 5 channel select"]
pub type Sel5R = crate::FieldReader;
#[doc = "Field `SEL5` writer - sequence 5 channel select"]
pub type Sel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL6` reader - sequence 6 channel select"]
pub type Sel6R = crate::FieldReader;
#[doc = "Field `SEL6` writer - sequence 6 channel select"]
pub type Sel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL7` reader - sequence 7 channel select"]
pub type Sel7R = crate::FieldReader;
#[doc = "Field `SEL7` writer - sequence 7 channel select"]
pub type Sel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sequence 0 channel select"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - sequence 1 channel select"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - sequence 2 channel select"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - sequence 3 channel select"]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - sequence 4 channel select"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - sequence 5 channel select"]
    #[inline(always)]
    pub fn sel5(&self) -> Sel5R {
        Sel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - sequence 6 channel select"]
    #[inline(always)]
    pub fn sel6(&self) -> Sel6R {
        Sel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - sequence 7 channel select"]
    #[inline(always)]
    pub fn sel7(&self) -> Sel7R {
        Sel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sequence 0 channel select"]
    #[inline(always)]
    pub fn sel0(&mut self) -> Sel0W<'_, Seqr0Spec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - sequence 1 channel select"]
    #[inline(always)]
    pub fn sel1(&mut self) -> Sel1W<'_, Seqr0Spec> {
        Sel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - sequence 2 channel select"]
    #[inline(always)]
    pub fn sel2(&mut self) -> Sel2W<'_, Seqr0Spec> {
        Sel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - sequence 3 channel select"]
    #[inline(always)]
    pub fn sel3(&mut self) -> Sel3W<'_, Seqr0Spec> {
        Sel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - sequence 4 channel select"]
    #[inline(always)]
    pub fn sel4(&mut self) -> Sel4W<'_, Seqr0Spec> {
        Sel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - sequence 5 channel select"]
    #[inline(always)]
    pub fn sel5(&mut self) -> Sel5W<'_, Seqr0Spec> {
        Sel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - sequence 6 channel select"]
    #[inline(always)]
    pub fn sel6(&mut self) -> Sel6W<'_, Seqr0Spec> {
        Sel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - sequence 7 channel select"]
    #[inline(always)]
    pub fn sel7(&mut self) -> Sel7W<'_, Seqr0Spec> {
        Sel7W::new(self, 28)
    }
}
#[doc = "sequence0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqr0Spec;
impl crate::RegisterSpec for Seqr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr0::R`](R) reader structure"]
impl crate::Readable for Seqr0Spec {}
#[doc = "`write(|w| ..)` method takes [`seqr0::W`](W) writer structure"]
impl crate::Writable for Seqr0Spec {
    type Safety = crate::Unsafe;
}
