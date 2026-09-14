#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `DMAC1_HANDSHAKE3_SEL` reader - dmac1 handshake 3 selection"]
pub type Dmac1Handshake3SelR = crate::FieldReader;
#[doc = "Field `DMAC1_HANDSHAKE3_SEL` writer - dmac1 handshake 3 selection"]
pub type Dmac1Handshake3SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAC1_HANDSHAKE2_SEL` reader - dmac1 handshake 2 selection"]
pub type Dmac1Handshake2SelR = crate::FieldReader;
#[doc = "Field `DMAC1_HANDSHAKE2_SEL` writer - dmac1 handshake 2 selection"]
pub type Dmac1Handshake2SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAC1_HANDSHAKE1_SEL` reader - dmac1 handshake 1 selection"]
pub type Dmac1Handshake1SelR = crate::FieldReader;
#[doc = "Field `DMAC1_HANDSHAKE1_SEL` writer - dmac1 handshake 1 selection"]
pub type Dmac1Handshake1SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAC1_HANDSHAKE0_SEL` reader - dmac1 handshake 0 selection"]
pub type Dmac1Handshake0SelR = crate::FieldReader;
#[doc = "Field `DMAC1_HANDSHAKE0_SEL` writer - dmac1 handshake 0 selection"]
pub type Dmac1Handshake0SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - dmac1 handshake 3 selection"]
    #[inline(always)]
    pub fn dmac1_handshake3_sel(&self) -> Dmac1Handshake3SelR {
        Dmac1Handshake3SelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - dmac1 handshake 2 selection"]
    #[inline(always)]
    pub fn dmac1_handshake2_sel(&self) -> Dmac1Handshake2SelR {
        Dmac1Handshake2SelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - dmac1 handshake 1 selection"]
    #[inline(always)]
    pub fn dmac1_handshake1_sel(&self) -> Dmac1Handshake1SelR {
        Dmac1Handshake1SelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - dmac1 handshake 0 selection"]
    #[inline(always)]
    pub fn dmac1_handshake0_sel(&self) -> Dmac1Handshake0SelR {
        Dmac1Handshake0SelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - dmac1 handshake 3 selection"]
    #[inline(always)]
    pub fn dmac1_handshake3_sel(&mut self) -> Dmac1Handshake3SelW<'_, Cr1Spec> {
        Dmac1Handshake3SelW::new(self, 0)
    }
    #[doc = "Bits 8:13 - dmac1 handshake 2 selection"]
    #[inline(always)]
    pub fn dmac1_handshake2_sel(&mut self) -> Dmac1Handshake2SelW<'_, Cr1Spec> {
        Dmac1Handshake2SelW::new(self, 8)
    }
    #[doc = "Bits 16:21 - dmac1 handshake 1 selection"]
    #[inline(always)]
    pub fn dmac1_handshake1_sel(&mut self) -> Dmac1Handshake1SelW<'_, Cr1Spec> {
        Dmac1Handshake1SelW::new(self, 16)
    }
    #[doc = "Bits 24:29 - dmac1 handshake 0 selection"]
    #[inline(always)]
    pub fn dmac1_handshake0_sel(&mut self) -> Dmac1Handshake0SelW<'_, Cr1Spec> {
        Dmac1Handshake0SelW::new(self, 24)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
