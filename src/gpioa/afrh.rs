#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AF8` reader - pin8 function selection"]
pub type Af8R = crate::FieldReader;
#[doc = "Field `AF8` writer - pin8 function selection"]
pub type Af8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF9` reader - pin9 function selection"]
pub type Af9R = crate::FieldReader;
#[doc = "Field `AF9` writer - pin9 function selection"]
pub type Af9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF10` reader - pin10 function selection"]
pub type Af10R = crate::FieldReader;
#[doc = "Field `AF10` writer - pin10 function selection"]
pub type Af10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF11` reader - pin11 function selection"]
pub type Af11R = crate::FieldReader;
#[doc = "Field `AF11` writer - pin11 function selection"]
pub type Af11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF12` reader - pin12 function selection"]
pub type Af12R = crate::FieldReader;
#[doc = "Field `AF12` writer - pin12 function selection"]
pub type Af12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF13` reader - pin13 function selection"]
pub type Af13R = crate::FieldReader;
#[doc = "Field `AF13` writer - pin13 function selection"]
pub type Af13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF14` reader - pin14 function selection"]
pub type Af14R = crate::FieldReader;
#[doc = "Field `AF14` writer - pin14 function selection"]
pub type Af14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF15` reader - pin15 function selection"]
pub type Af15R = crate::FieldReader;
#[doc = "Field `AF15` writer - pin15 function selection"]
pub type Af15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - pin8 function selection"]
    #[inline(always)]
    pub fn af8(&self) -> Af8R {
        Af8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pin9 function selection"]
    #[inline(always)]
    pub fn af9(&self) -> Af9R {
        Af9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pin10 function selection"]
    #[inline(always)]
    pub fn af10(&self) -> Af10R {
        Af10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pin11 function selection"]
    #[inline(always)]
    pub fn af11(&self) -> Af11R {
        Af11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - pin12 function selection"]
    #[inline(always)]
    pub fn af12(&self) -> Af12R {
        Af12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - pin13 function selection"]
    #[inline(always)]
    pub fn af13(&self) -> Af13R {
        Af13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - pin14 function selection"]
    #[inline(always)]
    pub fn af14(&self) -> Af14R {
        Af14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - pin15 function selection"]
    #[inline(always)]
    pub fn af15(&self) -> Af15R {
        Af15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pin8 function selection"]
    #[inline(always)]
    pub fn af8(&mut self) -> Af8W<'_, AfrhSpec> {
        Af8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - pin9 function selection"]
    #[inline(always)]
    pub fn af9(&mut self) -> Af9W<'_, AfrhSpec> {
        Af9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - pin10 function selection"]
    #[inline(always)]
    pub fn af10(&mut self) -> Af10W<'_, AfrhSpec> {
        Af10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - pin11 function selection"]
    #[inline(always)]
    pub fn af11(&mut self) -> Af11W<'_, AfrhSpec> {
        Af11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - pin12 function selection"]
    #[inline(always)]
    pub fn af12(&mut self) -> Af12W<'_, AfrhSpec> {
        Af12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - pin13 function selection"]
    #[inline(always)]
    pub fn af13(&mut self) -> Af13W<'_, AfrhSpec> {
        Af13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - pin14 function selection"]
    #[inline(always)]
    pub fn af14(&mut self) -> Af14W<'_, AfrhSpec> {
        Af14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - pin15 function selection"]
    #[inline(always)]
    pub fn af15(&mut self) -> Af15W<'_, AfrhSpec> {
        Af15W::new(self, 28)
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
