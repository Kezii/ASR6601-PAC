#[doc = "Register `CCER` reader"]
pub type R = crate::R<CcerSpec>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CcerSpec>;
#[doc = "Field `CC0E` reader - Cc0e"]
pub type Cc0eR = crate::BitReader;
#[doc = "Field `CC0E` writer - Cc0e"]
pub type Cc0eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0P` reader - Cc0p"]
pub type Cc0pR = crate::BitReader;
#[doc = "Field `CC0P` writer - Cc0p"]
pub type Cc0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0NP` reader - Cc0np"]
pub type Cc0npR = crate::BitReader;
#[doc = "Field `CC0NP` writer - Cc0np"]
pub type Cc0npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1E` reader - Cc1e"]
pub type Cc1eR = crate::BitReader;
#[doc = "Field `CC1E` writer - Cc1e"]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - Cc1p"]
pub type Cc1pR = crate::BitReader;
#[doc = "Field `CC1P` writer - Cc1p"]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NP` reader - Cc1np"]
pub type Cc1npR = crate::BitReader;
#[doc = "Field `CC1NP` writer - Cc1np"]
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - Cc2e"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - Cc2e"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Cc2p"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - Cc2p"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Cc2np"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - Cc2np"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Cc3e"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - Cc3e"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Cc3p"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - Cc3p"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Cc3np"]
pub type Cc3npR = crate::BitReader;
#[doc = "Field `CC3NP` writer - Cc3np"]
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cc0e"]
    #[inline(always)]
    pub fn cc0e(&self) -> Cc0eR {
        Cc0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cc0p"]
    #[inline(always)]
    pub fn cc0p(&self) -> Cc0pR {
        Cc0pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Cc0np"]
    #[inline(always)]
    pub fn cc0np(&self) -> Cc0npR {
        Cc0npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cc1e"]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cc1p"]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Cc1np"]
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cc2e"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cc2p"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Cc2np"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cc3e"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cc3p"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Cc3np"]
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cc0e"]
    #[inline(always)]
    pub fn cc0e(&mut self) -> Cc0eW<'_, CcerSpec> {
        Cc0eW::new(self, 0)
    }
    #[doc = "Bit 1 - Cc0p"]
    #[inline(always)]
    pub fn cc0p(&mut self) -> Cc0pW<'_, CcerSpec> {
        Cc0pW::new(self, 1)
    }
    #[doc = "Bit 3 - Cc0np"]
    #[inline(always)]
    pub fn cc0np(&mut self) -> Cc0npW<'_, CcerSpec> {
        Cc0npW::new(self, 3)
    }
    #[doc = "Bit 4 - Cc1e"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> Cc1eW<'_, CcerSpec> {
        Cc1eW::new(self, 4)
    }
    #[doc = "Bit 5 - Cc1p"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<'_, CcerSpec> {
        Cc1pW::new(self, 5)
    }
    #[doc = "Bit 7 - Cc1np"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> Cc1npW<'_, CcerSpec> {
        Cc1npW::new(self, 7)
    }
    #[doc = "Bit 8 - Cc2e"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<'_, CcerSpec> {
        Cc2eW::new(self, 8)
    }
    #[doc = "Bit 9 - Cc2p"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<'_, CcerSpec> {
        Cc2pW::new(self, 9)
    }
    #[doc = "Bit 11 - Cc2np"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> Cc2npW<'_, CcerSpec> {
        Cc2npW::new(self, 11)
    }
    #[doc = "Bit 12 - Cc3e"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> Cc3eW<'_, CcerSpec> {
        Cc3eW::new(self, 12)
    }
    #[doc = "Bit 13 - Cc3p"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> Cc3pW<'_, CcerSpec> {
        Cc3pW::new(self, 13)
    }
    #[doc = "Bit 15 - Cc3np"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> Cc3npW<'_, CcerSpec> {
        Cc3npW::new(self, 15)
    }
}
#[doc = "TIMER capture/compare enable register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerSpec;
impl crate::RegisterSpec for CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CcerSpec {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CcerSpec {
    type Safety = crate::Unsafe;
}
