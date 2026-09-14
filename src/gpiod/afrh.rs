#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AF8` reader - pin8 function selection"]
pub type Af8R = crate::FieldReader;
#[doc = "Field `AF8` writer - pin8 function selection"]
pub type Af8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF9` reader - pin9 function selection"]
pub type Af9R = crate::FieldReader;
#[doc = "Field `AF9` writer - pin9 function selection"]
pub type Af9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF10` reader - pin10 function selection"]
pub type Af10R = crate::FieldReader;
#[doc = "Field `AF10` writer - pin10 function selection"]
pub type Af10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF11` reader - pin11 function selection"]
pub type Af11R = crate::FieldReader;
#[doc = "Field `AF11` writer - pin11 function selection"]
pub type Af11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF12` reader - pin12 function selection"]
pub type Af12R = crate::FieldReader;
#[doc = "Field `AF12` writer - pin12 function selection"]
pub type Af12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF13` reader - pin13 function selection"]
pub type Af13R = crate::FieldReader;
#[doc = "Field `AF13` writer - pin13 function selection"]
pub type Af13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF14` reader - pin14 function selection"]
pub type Af14R = crate::FieldReader;
#[doc = "Field `AF14` writer - pin14 function selection"]
pub type Af14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AF15` reader - pin15 function selection"]
pub type Af15R = crate::FieldReader;
#[doc = "Field `AF15` writer - pin15 function selection"]
pub type Af15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - pin8 function selection"]
    #[inline(always)]
    pub fn af8(&self) -> Af8R {
        Af8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - pin9 function selection"]
    #[inline(always)]
    pub fn af9(&self) -> Af9R {
        Af9R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - pin10 function selection"]
    #[inline(always)]
    pub fn af10(&self) -> Af10R {
        Af10R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - pin11 function selection"]
    #[inline(always)]
    pub fn af11(&self) -> Af11R {
        Af11R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - pin12 function selection"]
    #[inline(always)]
    pub fn af12(&self) -> Af12R {
        Af12R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - pin13 function selection"]
    #[inline(always)]
    pub fn af13(&self) -> Af13R {
        Af13R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - pin14 function selection"]
    #[inline(always)]
    pub fn af14(&self) -> Af14R {
        Af14R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - pin15 function selection"]
    #[inline(always)]
    pub fn af15(&self) -> Af15R {
        Af15R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - pin8 function selection"]
    #[inline(always)]
    pub fn af8(&mut self) -> Af8W<'_, AfrhSpec> {
        Af8W::new(self, 0)
    }
    #[doc = "Bits 3:5 - pin9 function selection"]
    #[inline(always)]
    pub fn af9(&mut self) -> Af9W<'_, AfrhSpec> {
        Af9W::new(self, 3)
    }
    #[doc = "Bits 6:8 - pin10 function selection"]
    #[inline(always)]
    pub fn af10(&mut self) -> Af10W<'_, AfrhSpec> {
        Af10W::new(self, 6)
    }
    #[doc = "Bits 9:11 - pin11 function selection"]
    #[inline(always)]
    pub fn af11(&mut self) -> Af11W<'_, AfrhSpec> {
        Af11W::new(self, 9)
    }
    #[doc = "Bits 12:14 - pin12 function selection"]
    #[inline(always)]
    pub fn af12(&mut self) -> Af12W<'_, AfrhSpec> {
        Af12W::new(self, 12)
    }
    #[doc = "Bits 15:17 - pin13 function selection"]
    #[inline(always)]
    pub fn af13(&mut self) -> Af13W<'_, AfrhSpec> {
        Af13W::new(self, 15)
    }
    #[doc = "Bits 18:20 - pin14 function selection"]
    #[inline(always)]
    pub fn af14(&mut self) -> Af14W<'_, AfrhSpec> {
        Af14W::new(self, 18)
    }
    #[doc = "Bits 21:23 - pin15 function selection"]
    #[inline(always)]
    pub fn af15(&mut self) -> Af15W<'_, AfrhSpec> {
        Af15W::new(self, 21)
    }
}
#[doc = "alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
