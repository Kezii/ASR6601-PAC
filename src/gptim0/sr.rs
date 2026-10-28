#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UIF` reader - Update event flag; write 0 to clear"]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - Update event flag; write 0 to clear"]
pub type UifW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC0IF` reader - Channel 0 capture/compare event flag; write 0 to clear"]
pub type Cc0ifR = crate::BitReader;
#[doc = "Field `CC0IF` writer - Channel 0 capture/compare event flag; write 0 to clear"]
pub type Cc0ifW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC1IF` reader - Channel 1 capture/compare event flag; write 0 to clear"]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC1IF` writer - Channel 1 capture/compare event flag; write 0 to clear"]
pub type Cc1ifW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC2IF` reader - Channel 2 capture/compare event flag; write 0 to clear"]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC2IF` writer - Channel 2 capture/compare event flag; write 0 to clear"]
pub type Cc2ifW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC3IF` reader - Channel 3 capture/compare event flag; write 0 to clear"]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `CC3IF` writer - Channel 3 capture/compare event flag; write 0 to clear"]
pub type Cc3ifW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `TIF` reader - Trigger event flag; write 0 to clear"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - Trigger event flag; write 0 to clear"]
pub type TifW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC0OF` reader - Channel 0 overcapture flag; write 0 to clear"]
pub type Cc0ofR = crate::BitReader;
#[doc = "Field `CC0OF` writer - Channel 0 overcapture flag; write 0 to clear"]
pub type Cc0ofW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC1OF` reader - Channel 1 overcapture flag; write 0 to clear"]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - Channel 1 overcapture flag; write 0 to clear"]
pub type Cc1ofW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC2OF` reader - Channel 2 overcapture flag; write 0 to clear"]
pub type Cc2ofR = crate::BitReader;
#[doc = "Field `CC2OF` writer - Channel 2 overcapture flag; write 0 to clear"]
pub type Cc2ofW<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CC3OF` reader - Channel 3 overcapture flag; write 0 to clear"]
pub type Cc3ofR = crate::BitReader;
#[doc = "Field `CC3OF` writer - Channel 3 overcapture flag; write 0 to clear"]
pub type Cc3ofW<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update event flag; write 0 to clear"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc0if(&self) -> Cc0ifR {
        Cc0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger event flag; write 0 to clear"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc0of(&self) -> Cc0ofR {
        Cc0ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update event flag; write 0 to clear"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<'_, SrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc0if(&mut self) -> Cc0ifW<'_, SrSpec> {
        Cc0ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<'_, SrSpec> {
        Cc1ifW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<'_, SrSpec> {
        Cc2ifW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare event flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<'_, SrSpec> {
        Cc3ifW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger event flag; write 0 to clear"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, SrSpec> {
        TifW::new(self, 6)
    }
    #[doc = "Bit 9 - Channel 0 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc0of(&mut self) -> Cc0ofW<'_, SrSpec> {
        Cc0ofW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<'_, SrSpec> {
        Cc1ofW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> Cc2ofW<'_, SrSpec> {
        Cc2ofW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 overcapture flag; write 0 to clear"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> Cc3ofW<'_, SrSpec> {
        Cc3ofW::new(self, 12)
    }
}
#[doc = "GPTIM status register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1e5f;
}
