#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `DMAC0_HANDSHAKE3_SEL` reader - dmac0 handshake 3 selection"]
pub type Dmac0Handshake3SelR = crate::FieldReader;
#[doc = "Field `DMAC0_HANDSHAKE3_SEL` writer - dmac0 handshake 3 selection"]
pub type Dmac0Handshake3SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAC0_HANDSHAKE2_SEL` reader - dmac0 handshake 2 selection"]
pub type Dmac0Handshake2SelR = crate::FieldReader;
#[doc = "Field `DMAC0_HANDSHAKE2_SEL` writer - dmac0 handshake 2 selection"]
pub type Dmac0Handshake2SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAC0_HANDSHAKE1_SEL` reader - dmac0 handshake 1 selection"]
pub type Dmac0Handshake1SelR = crate::FieldReader;
#[doc = "Field `DMAC0_HANDSHAKE1_SEL` writer - dmac0 handshake 1 selection"]
pub type Dmac0Handshake1SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAC0_HANDSHAKE0_SEL` reader - dmac0 handshake 0 selection"]
pub type Dmac0Handshake0SelR = crate::FieldReader;
#[doc = "Field `DMAC0_HANDSHAKE0_SEL` writer - dmac0 handshake 0 selection"]
pub type Dmac0Handshake0SelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - dmac0 handshake 3 selection"]
    #[inline(always)]
    pub fn dmac0_handshake3_sel(&self) -> Dmac0Handshake3SelR {
        Dmac0Handshake3SelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - dmac0 handshake 2 selection"]
    #[inline(always)]
    pub fn dmac0_handshake2_sel(&self) -> Dmac0Handshake2SelR {
        Dmac0Handshake2SelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - dmac0 handshake 1 selection"]
    #[inline(always)]
    pub fn dmac0_handshake1_sel(&self) -> Dmac0Handshake1SelR {
        Dmac0Handshake1SelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - dmac0 handshake 0 selection"]
    #[inline(always)]
    pub fn dmac0_handshake0_sel(&self) -> Dmac0Handshake0SelR {
        Dmac0Handshake0SelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - dmac0 handshake 3 selection"]
    #[inline(always)]
    pub fn dmac0_handshake3_sel(&mut self) -> Dmac0Handshake3SelW<'_, Cr0Spec> {
        Dmac0Handshake3SelW::new(self, 0)
    }
    #[doc = "Bits 8:13 - dmac0 handshake 2 selection"]
    #[inline(always)]
    pub fn dmac0_handshake2_sel(&mut self) -> Dmac0Handshake2SelW<'_, Cr0Spec> {
        Dmac0Handshake2SelW::new(self, 8)
    }
    #[doc = "Bits 16:21 - dmac0 handshake 1 selection"]
    #[inline(always)]
    pub fn dmac0_handshake1_sel(&mut self) -> Dmac0Handshake1SelW<'_, Cr0Spec> {
        Dmac0Handshake1SelW::new(self, 16)
    }
    #[doc = "Bits 24:29 - dmac0 handshake 0 selection"]
    #[inline(always)]
    pub fn dmac0_handshake0_sel(&mut self) -> Dmac0Handshake0SelW<'_, Cr0Spec> {
        Dmac0Handshake0SelW::new(self, 24)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
