#[doc = "Register `SEQR1` reader"]
pub type R = crate::R<Seqr1Spec>;
#[doc = "Register `SEQR1` writer"]
pub type W = crate::W<Seqr1Spec>;
#[doc = "Field `SEL8` reader - sequence 8 channel select"]
pub type Sel8R = crate::FieldReader;
#[doc = "Field `SEL8` writer - sequence 8 channel select"]
pub type Sel8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL9` reader - sequence 9 channel select"]
pub type Sel9R = crate::FieldReader;
#[doc = "Field `SEL9` writer - sequence 9 channel select"]
pub type Sel9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL10` reader - sequence 10 channel select"]
pub type Sel10R = crate::FieldReader;
#[doc = "Field `SEL10` writer - sequence 10 channel select"]
pub type Sel10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL11` reader - sequence 11 channel select"]
pub type Sel11R = crate::FieldReader;
#[doc = "Field `SEL11` writer - sequence 11 channel select"]
pub type Sel11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL12` reader - sequence 12 channel select"]
pub type Sel12R = crate::FieldReader;
#[doc = "Field `SEL12` writer - sequence 12 channel select"]
pub type Sel12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL13` reader - sequence 13 channel select"]
pub type Sel13R = crate::FieldReader;
#[doc = "Field `SEL13` writer - sequence 13 channel select"]
pub type Sel13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL14` reader - sequence 14 channel select"]
pub type Sel14R = crate::FieldReader;
#[doc = "Field `SEL14` writer - sequence 14 channel select"]
pub type Sel14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL15` reader - sequence 15 channel select"]
pub type Sel15R = crate::FieldReader;
#[doc = "Field `SEL15` writer - sequence 15 channel select"]
pub type Sel15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sequence 8 channel select"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - sequence 9 channel select"]
    #[inline(always)]
    pub fn sel9(&self) -> Sel9R {
        Sel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - sequence 10 channel select"]
    #[inline(always)]
    pub fn sel10(&self) -> Sel10R {
        Sel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - sequence 11 channel select"]
    #[inline(always)]
    pub fn sel11(&self) -> Sel11R {
        Sel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - sequence 12 channel select"]
    #[inline(always)]
    pub fn sel12(&self) -> Sel12R {
        Sel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - sequence 13 channel select"]
    #[inline(always)]
    pub fn sel13(&self) -> Sel13R {
        Sel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - sequence 14 channel select"]
    #[inline(always)]
    pub fn sel14(&self) -> Sel14R {
        Sel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - sequence 15 channel select"]
    #[inline(always)]
    pub fn sel15(&self) -> Sel15R {
        Sel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sequence 8 channel select"]
    #[inline(always)]
    pub fn sel8(&mut self) -> Sel8W<'_, Seqr1Spec> {
        Sel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - sequence 9 channel select"]
    #[inline(always)]
    pub fn sel9(&mut self) -> Sel9W<'_, Seqr1Spec> {
        Sel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - sequence 10 channel select"]
    #[inline(always)]
    pub fn sel10(&mut self) -> Sel10W<'_, Seqr1Spec> {
        Sel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - sequence 11 channel select"]
    #[inline(always)]
    pub fn sel11(&mut self) -> Sel11W<'_, Seqr1Spec> {
        Sel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - sequence 12 channel select"]
    #[inline(always)]
    pub fn sel12(&mut self) -> Sel12W<'_, Seqr1Spec> {
        Sel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - sequence 13 channel select"]
    #[inline(always)]
    pub fn sel13(&mut self) -> Sel13W<'_, Seqr1Spec> {
        Sel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - sequence 14 channel select"]
    #[inline(always)]
    pub fn sel14(&mut self) -> Sel14W<'_, Seqr1Spec> {
        Sel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - sequence 15 channel select"]
    #[inline(always)]
    pub fn sel15(&mut self) -> Sel15W<'_, Seqr1Spec> {
        Sel15W::new(self, 28)
    }
}
#[doc = "sequence1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqr1Spec;
impl crate::RegisterSpec for Seqr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr1::R`](R) reader structure"]
impl crate::Readable for Seqr1Spec {}
#[doc = "`write(|w| ..)` method takes [`seqr1::W`](W) writer structure"]
impl crate::Writable for Seqr1Spec {
    type Safety = crate::Unsafe;
}
