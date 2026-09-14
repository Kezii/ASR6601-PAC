#[doc = "Register `CR6` reader"]
pub type R = crate::R<Cr6Spec>;
#[doc = "Register `CR6` writer"]
pub type W = crate::W<Cr6Spec>;
#[doc = "Field `DMAC0_MASTER_SECURE_LOCK` reader - dmac0 master interface security lock"]
pub type Dmac0MasterSecureLockR = crate::BitReader;
#[doc = "Field `DMAC0_MASTER_SECURE_LOCK` writer - dmac0 master interface security lock"]
pub type Dmac0MasterSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC0_SLAVE_SECURE_LOCK` reader - dmac0 slave interface security lock"]
pub type Dmac0SlaveSecureLockR = crate::BitReader;
#[doc = "Field `DMAC0_SLAVE_SECURE_LOCK` writer - dmac0 slave interface security lock"]
pub type Dmac0SlaveSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAC_SECURE_LOCK` reader - sac security lock"]
pub type SacSecureLockR = crate::BitReader;
#[doc = "Field `SAC_SECURE_LOCK` writer - sac security lock"]
pub type SacSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_SECURE_LOCK` reader - sec security lock"]
pub type SecSecureLockR = crate::BitReader;
#[doc = "Field `SEC_SECURE_LOCK` writer - sec security lock"]
pub type SecSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANALOG_MAIN_SECURE_LOCK` reader - security lock for main domain configuration of afec"]
pub type AnalogMainSecureLockR = crate::FieldReader<u16>;
#[doc = "Field `ANALOG_MAIN_SECURE_LOCK` writer - security lock for main domain configuration of afec"]
pub type AnalogMainSecureLockW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RNGC_SECURE_LOCK` reader - rngc security lock"]
pub type RngcSecureLockR = crate::BitReader;
#[doc = "Field `RNGC_SECURE_LOCK` writer - rngc security lock"]
pub type RngcSecureLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - dmac0 master interface security lock"]
    #[inline(always)]
    pub fn dmac0_master_secure_lock(&self) -> Dmac0MasterSecureLockR {
        Dmac0MasterSecureLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - dmac0 slave interface security lock"]
    #[inline(always)]
    pub fn dmac0_slave_secure_lock(&self) -> Dmac0SlaveSecureLockR {
        Dmac0SlaveSecureLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sac security lock"]
    #[inline(always)]
    pub fn sac_secure_lock(&self) -> SacSecureLockR {
        SacSecureLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sec security lock"]
    #[inline(always)]
    pub fn sec_secure_lock(&self) -> SecSecureLockR {
        SecSecureLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - security lock for main domain configuration of afec"]
    #[inline(always)]
    pub fn analog_main_secure_lock(&self) -> AnalogMainSecureLockR {
        AnalogMainSecureLockR::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - rngc security lock"]
    #[inline(always)]
    pub fn rngc_secure_lock(&self) -> RngcSecureLockR {
        RngcSecureLockR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dmac0 master interface security lock"]
    #[inline(always)]
    pub fn dmac0_master_secure_lock(&mut self) -> Dmac0MasterSecureLockW<'_, Cr6Spec> {
        Dmac0MasterSecureLockW::new(self, 0)
    }
    #[doc = "Bit 1 - dmac0 slave interface security lock"]
    #[inline(always)]
    pub fn dmac0_slave_secure_lock(&mut self) -> Dmac0SlaveSecureLockW<'_, Cr6Spec> {
        Dmac0SlaveSecureLockW::new(self, 1)
    }
    #[doc = "Bit 2 - sac security lock"]
    #[inline(always)]
    pub fn sac_secure_lock(&mut self) -> SacSecureLockW<'_, Cr6Spec> {
        SacSecureLockW::new(self, 2)
    }
    #[doc = "Bit 3 - sec security lock"]
    #[inline(always)]
    pub fn sec_secure_lock(&mut self) -> SecSecureLockW<'_, Cr6Spec> {
        SecSecureLockW::new(self, 3)
    }
    #[doc = "Bits 5:14 - security lock for main domain configuration of afec"]
    #[inline(always)]
    pub fn analog_main_secure_lock(&mut self) -> AnalogMainSecureLockW<'_, Cr6Spec> {
        AnalogMainSecureLockW::new(self, 5)
    }
    #[doc = "Bit 15 - rngc security lock"]
    #[inline(always)]
    pub fn rngc_secure_lock(&mut self) -> RngcSecureLockW<'_, Cr6Spec> {
        RngcSecureLockW::new(self, 15)
    }
}
#[doc = "control register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`cr6::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr6Spec;
impl crate::RegisterSpec for Cr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr6::R`](R) reader structure"]
impl crate::Readable for Cr6Spec {}
#[doc = "`write(|w| ..)` method takes [`cr6::W`](W) writer structure"]
impl crate::Writable for Cr6Spec {
    type Safety = crate::Unsafe;
}
