#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AF0` reader - pin0 function selection"]
pub type Af0R = crate::FieldReader;
#[doc = "Field `AF0` writer - pin0 function selection"]
pub type Af0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF1` reader - pin1 function selection"]
pub type Af1R = crate::FieldReader;
#[doc = "Field `AF1` writer - pin1 function selection"]
pub type Af1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF2` reader - pin2 function selection"]
pub type Af2R = crate::FieldReader;
#[doc = "Field `AF2` writer - pin2 function selection"]
pub type Af2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF3` reader - pin3 function selection"]
pub type Af3R = crate::FieldReader;
#[doc = "Field `AF3` writer - pin3 function selection"]
pub type Af3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF4` reader - pin4 function selection"]
pub type Af4R = crate::FieldReader;
#[doc = "Field `AF4` writer - pin4 function selection"]
pub type Af4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF5` reader - pin5 function selection"]
pub type Af5R = crate::FieldReader;
#[doc = "Field `AF5` writer - pin5 function selection"]
pub type Af5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF6` reader - pin6 function selection"]
pub type Af6R = crate::FieldReader;
#[doc = "Field `AF6` writer - pin6 function selection"]
pub type Af6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF7` reader - pin7 function selection"]
pub type Af7R = crate::FieldReader;
#[doc = "Field `AF7` writer - pin7 function selection"]
pub type Af7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - pin0 function selection"]
    #[inline(always)]
    pub fn af0(&self) -> Af0R {
        Af0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pin1 function selection"]
    #[inline(always)]
    pub fn af1(&self) -> Af1R {
        Af1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pin2 function selection"]
    #[inline(always)]
    pub fn af2(&self) -> Af2R {
        Af2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pin3 function selection"]
    #[inline(always)]
    pub fn af3(&self) -> Af3R {
        Af3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - pin4 function selection"]
    #[inline(always)]
    pub fn af4(&self) -> Af4R {
        Af4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - pin5 function selection"]
    #[inline(always)]
    pub fn af5(&self) -> Af5R {
        Af5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - pin6 function selection"]
    #[inline(always)]
    pub fn af6(&self) -> Af6R {
        Af6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - pin7 function selection"]
    #[inline(always)]
    pub fn af7(&self) -> Af7R {
        Af7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pin0 function selection"]
    #[inline(always)]
    pub fn af0(&mut self) -> Af0W<'_, AfrlSpec> {
        Af0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - pin1 function selection"]
    #[inline(always)]
    pub fn af1(&mut self) -> Af1W<'_, AfrlSpec> {
        Af1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - pin2 function selection"]
    #[inline(always)]
    pub fn af2(&mut self) -> Af2W<'_, AfrlSpec> {
        Af2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - pin3 function selection"]
    #[inline(always)]
    pub fn af3(&mut self) -> Af3W<'_, AfrlSpec> {
        Af3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - pin4 function selection"]
    #[inline(always)]
    pub fn af4(&mut self) -> Af4W<'_, AfrlSpec> {
        Af4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - pin5 function selection"]
    #[inline(always)]
    pub fn af5(&mut self) -> Af5W<'_, AfrlSpec> {
        Af5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - pin6 function selection"]
    #[inline(always)]
    pub fn af6(&mut self) -> Af6W<'_, AfrlSpec> {
        Af6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - pin7 function selection"]
    #[inline(always)]
    pub fn af7(&mut self) -> Af7W<'_, AfrlSpec> {
        Af7W::new(self, 28)
    }
}
#[doc = "alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
